//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 693/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk693<F: Float>(t168: F, t270: F, t5589: F, t153: F, t274: F, t4573: F, t156: F, t242: F, t245: F, t4550: F, t4552: F, t4554: F, t4557: F, t4563: F, t4566: F, t4568: F, t4600: F, t4867: F, t5569: F, t5574: F, t5577: F, t5580: F, t5582: F, t5585: F, t5588: F) -> F {
    let t5592 = F::cast_from(0.19455129084526283664e0_f64) * t168 * t5589 * t270;
    let t5595 = F::cast_from(0.4429070076315393047e1_f64) * t153 * t4573 * t274;
    let t5596 = -t4550 + F::cast_from(0.25128846160651320563e0_f64) * t4552 + F::cast_from(0.50257692321302641125e0_f64) * t4554 + t4557 - F::cast_from(0.83762820535504401876e-1_f64) * t4563 * t242 - F::cast_from(0.25128846160651320563e0_f64) * t4566 - F::cast_from(0.25128846160651320563e0_f64) * t4568 - t4600 + F::cast_from(0.42708890021612718669e0_f64) * t153 * t156 * t4867 - F::cast_from(0.11938374665504764976e-1_f64) * t168 * t245 * t5569 - F::cast_from(0.15917832887339686635e0_f64) * t5574 + F::cast_from(0.59691873327523824879e-1_f64) * t5577 - F::cast_from(0.17083556008645087467e1_f64) * t5580 - F::cast_from(0.50257692321302641125e0_f64) * t5582 + F::cast_from(0.39861630686838537423e1_f64) * t5585 + t5588 + t5592 - t5595;
    t5596
}
