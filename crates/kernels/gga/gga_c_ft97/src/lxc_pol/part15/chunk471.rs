//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 471/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk471<F: Float>(t2221: F, t4823: F, t1053: F, t920: F, t2211: F, t2210: F, t167: F, t4458: F, t569: F, t1901: F, t2164: F, t28: F, t3460: F, t3489: F, t3545: F, t3551: F, t446: F, t4726: F, t4730: F, t4735: F, t4739: F, t4743: F, t4747: F, t4792: F, t4807: F, t4811: F, t4815: F, t4819: F, t89: F) -> (F, F, F, F, F) {
    let t4824 = t2221 * t4823;
    let t4827 = t920 * t1053;
    let t4828 = t2211 * t4827;
    let t4829 = t2210 * t4828;
    let t4833 = t569 * t167 * t4458;
    let t4837 = F::new(2.0) / F::new(3.0) * t446 * t4726 + F::new(2.0) / F::new(3.0) * t446 * t4730 + F::new(2.0) / F::new(3.0) * t446 * t4735 - F::new(2.0) / F::new(9.0) * t446 * t4739 - t446 * t4743 / F::new(9.0) - F::new(2.0) / F::new(27.0) * t446 * t4747 + t2164 - F::new(2.0) / F::new(9.0) * t3489 + F::new(2.0) / F::new(9.0) * t3551 + F::new(2.0) / F::new(9.0) * t3545 + t89 * t28 * t4792 / F::new(3.0) - t446 * t4807 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t446 * t4811 - F::new(2.0) / F::new(3.0) * t446 * t4815 - t446 * t4819 / F::new(3.0) + F::new(2.0) / F::new(9.0) * t1901 * t4824 + F::new(2.0) / F::new(9.0) * t1901 * t4829 + F::new(2.0) / F::new(9.0) * t446 * t4833 + F::new(2.0) / F::new(27.0) * t3460;
    (t4824, t4828, t4829, t4833, t4837)
}
