//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta358 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1478;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1479;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1480;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta358(t13242: f64, t4180: f64, t4182: f64, t4181: f64, t9632: f64, t2642: f64, t4166: f64, t2617: f64, t4177: f64, t2628: f64, t836: f64, t812: f64, t4184: f64, t242: f64, t9972: f64, t2631: f64, t9975: f64, t13225: f64, t13231: f64, t13234: f64, t13237: f64, t2643: f64, t2649: f64, t4178: f64, t4191: f64, t4240: f64, t9639: f64, t9642: f64, t9668: f64, t9672: f64, t9675: f64, t9679: f64, t9986: f64, t9988: f64, t9994: f64, t2639: f64, t4236: f64, t1512: f64, t9674: f64, t2638: f64, t831: f64, t2629: f64, t4250: f64, t9638: f64, t1495: f64, t210: f64, t2379: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13244, t13248, t13251, t13254, t13258) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1478(t13242, t4180, t4182, t4181, t9632, t2642, t4166, t2617, t4177, t2628, t836, t812);
        let (t13263, t13265, t13268) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1479(t13258, t4184, t242, t9972, t812, t2631, t9975, t4180, t4181, t13225, t13231, t13234, t13237, t13244, t13248, t13251, t13254, t2643, t2649, t4178, t4191, t4240, t9639, t9642, t9668, t9672, t9675, t9679, t9986, t9988, t9994);
        let (t13275, t13277, t13280, t13283, t13287, t13289) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1480(t2639, t4236, t1512, t9674, t2638, t4166, t831, t2629, t4250, t9638, t1495, t210, t2379);
    (t13244, t13248, t13263, t13265, t13268, t13275, t13277, t13280, t13283, t13287, t13289)
}
