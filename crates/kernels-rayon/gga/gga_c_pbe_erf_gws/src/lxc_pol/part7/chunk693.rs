//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 693/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk693(t168: f64, t270: f64, t5589: f64, t153: f64, t274: f64, t4573: f64, t156: f64, t242: f64, t245: f64, t4550: f64, t4552: f64, t4554: f64, t4557: f64, t4563: f64, t4566: f64, t4568: f64, t4600: f64, t4867: f64, t5569: f64, t5574: f64, t5577: f64, t5580: f64, t5582: f64, t5585: f64, t5588: f64) -> f64 {
    let t5592 = 0.19455129084526283664e0_f64 * t168 * t5589 * t270;
    let t5595 = 0.4429070076315393047e1_f64 * t153 * t4573 * t274;
    let t5596 = -t4550 + 0.25128846160651320563e0_f64 * t4552 + 0.50257692321302641125e0_f64 * t4554 + t4557 - 0.83762820535504401876e-1_f64 * t4563 * t242 - 0.25128846160651320563e0_f64 * t4566 - 0.25128846160651320563e0_f64 * t4568 - t4600 + 0.42708890021612718669e0_f64 * t153 * t156 * t4867 - 0.11938374665504764976e-1_f64 * t168 * t245 * t5569 - 0.15917832887339686635e0_f64 * t5574 + 0.59691873327523824879e-1_f64 * t5577 - 0.17083556008645087467e1_f64 * t5580 - 0.50257692321302641125e0_f64 * t5582 + 0.39861630686838537423e1_f64 * t5585 + t5588 + t5592 - t5595;
    t5596
}
