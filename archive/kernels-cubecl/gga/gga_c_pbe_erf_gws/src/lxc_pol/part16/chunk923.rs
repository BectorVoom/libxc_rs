//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 923/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk923<F: Float>(t168: F, t2831: F, t703: F, t1072: F, t1472: F, t4550: F, t4552: F, t4554: F, t4557: F, t4566: F, t4568: F, t4600: F, t5574: F, t5577: F, t5582: F, t5588: F, t5595: F) -> F {
    let t8064 = F::cast_from(0.39794582218349216586e-1_f64) * t168 * t703 * t2831;
    let t8066 = t168 * t1472 * t1072;
    let t8075 = -t5595 + t8064 - F::cast_from(0.53059442957798955448e-1_f64) * t8066 - t4550 + F::cast_from(0.83762820535504401876e-1_f64) * t4552 + F::cast_from(0.33505128214201760751e0_f64) * t4554 + t4557 - F::cast_from(0.83762820535504401876e-1_f64) * t4566 - F::cast_from(0.16752564107100880375e0_f64) * t4568 - t4600 - F::cast_from(0.1061188859155979109e0_f64) * t5574 + F::cast_from(0.19897291109174608293e-1_f64) * t5577 - F::cast_from(0.3350512821420176075e0_f64) * t5582 + t5588;
    t8075
}
