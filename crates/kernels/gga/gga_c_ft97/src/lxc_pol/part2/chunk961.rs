//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 961/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk961<F: Float>(t14657: F, t14683: F, t14655: F, t14662: F, t14666: F, t14669: F, t14673: F, t14676: F, t14680: F, t14688: F, t14692: F, t14715: F) -> (F, F) {
    let t14929 = F::new(2.0) / F::new(9.0) * t14657;
    let t14936 = F::new(4.0) / F::new(3.0) * t14683;
    let t14939 = F::new(2.0) / F::new(9.0) * t14655 - t14929 + F::new(2.0) / F::new(3.0) * t14662 + t14666 / F::new(3.0) + F::new(4.0) / F::new(3.0) * t14669 - F::new(2.0) / F::new(3.0) * t14673 - F::new(2.0) * t14676 - F::new(4.0) / F::new(3.0) * t14680 - t14936 + F::new(4.0) / F::new(9.0) * t14688 - F::new(4.0) / F::new(3.0) * t14692;
    let t14946 = F::new(4.0) / F::new(27.0) * t14715;
    (t14939, t14946)
}
