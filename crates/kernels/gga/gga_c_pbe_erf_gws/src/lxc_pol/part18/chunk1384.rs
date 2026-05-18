//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1384/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1384<F: Float>(t14583: F, t50998: F, t53860: F, t14404: F, t26958: F, t1177: F, t1178: F, t12099: F, t371: F, t52020: F, t52036: F, t53464: F, t54717: F, t54727: F, t54729: F, t54731: F, t55984: F, t55986: F, t57740: F, t57745: F, t57747: F, t57751: F, t6793: F, t8629: F) -> F {
    let t57755 = t50998 * t53860 * t14583;
    let t57757 = t26958 * t14404;
    let t57764 = t1177 * t371 * t1178 * t12099;
    let t57767 = -t57740 / F::new(3072.0) + t57745 / F::new(1536.0) - t57747 / F::new(16.0) - t6793 * t57751 / F::new(16.0) + t57755 / F::new(192.0) - F::new(7.0) / F::new(72.0) * t57757 + t8629 * t53464 / F::new(48.0) + t54717 - F::new(35.0) / F::new(432.0) * t52020 - t55984 - t55986 - t57764 / F::new(3072.0) + t54727 + t54729 + t54731 + F::new(35.0) / F::new(432.0) * t52036;
    t57767
}
