//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1837/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1837<F: Float>(t1437: F, t22953: F, t4003: F, t47442: F, t47454: F, t49432: F, t5735: F, t5745: F, t75274: F, t820: F, t86634: F, t86639: F, t86643: F, t86647: F, t86654: F, t92064: F) -> F {
    let t92409 = t47442 + F::cast_from(0.43902994552903410657e-1_f64) * t75274 - F::cast_from(0.13170898365871023197e0_f64) * t86634 - F::cast_from(0.39029762157531132076e-1_f64) * t86639 + F::cast_from(0.65854491829355115985e-1_f64) * t86643 - F::cast_from(0.13170898365871023197e0_f64) * t86647 + F::cast_from(0.65854491829355115985e-1_f64) * t86654 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t1437 * t92064 - F::cast_from(0.18505311230957427423e-1_f64) * t49432 + t47454 + F::cast_from(0.52683593463484092788e1_f64) * t5745 * t5735 * t4003 * t22953;
    t92409
}
