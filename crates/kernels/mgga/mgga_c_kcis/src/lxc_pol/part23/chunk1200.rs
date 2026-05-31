//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1200/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1200<F: Float>(t28314: F, t28317: F, t28320: F, t28323: F, t27722: F, t28901: F, t91769: F, t91772: F, t91773: F, t91776: F, t91777: F, t91778: F, t91781: F, t91785: F, t95271: F, t95275: F, t97622: F) -> F {
    let t97623 = t28314 / F::cast_from(8.0_f64);
    let t97624 = t28317 / F::cast_from(8.0_f64);
    let t97625 = t28320 / F::cast_from(8.0_f64);
    let t97626 = t28323 / F::cast_from(8.0_f64);
    let t97627 = -t91769 + t91772 + t91773 + t97622 + t95271 - t91776 - t97623 + t91777 + t28901 - t91778 + t95275 - t97624 - t91781 + t97625 - t91785 - t97626 - t27722;
    t97627
}
