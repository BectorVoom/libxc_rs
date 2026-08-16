//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta382 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1572;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1573;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1574;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta382(t14238: f64, t14487: f64, t360: f64, t1021: f64, t248: f64, t3053: f64, t4644: f64, t10422: f64, t4578: f64, t3070: f64, t1603: f64, t3030: f64, t3032: f64, t3129: f64, t3038: f64, t1020: f64, t10937: f64, t10962: f64, t10982: f64, t10985: f64, t10994: f64, t11003: f64, t14235: f64, t1618: f64, t3043: f64, t3057: f64, t3064: f64, t3114: f64, t3123: f64, t3134: f64, t4579: f64, t4641: f64, t4652: f64, t13953: f64, t14004: f64, t14050: f64, t14074: f64, t14120: f64, t14170: f64, t14233: f64, t349: f64, t225: f64, t4658: f64, t1625: f64, t3020: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14488, t14491, t14495, t14501, t14503, t14506) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1572(t14238, t14487, t360, t1021, t248, t3053, t4644, t10422, t4578, t3070, t1603, t3030);
        let (t14507, t14523) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1573(t14506, t3032, t3129, t3038, t1020, t10937, t10962, t10982, t10985, t10994, t11003, t14235, t14491, t14495, t14503, t1618, t3043, t3057, t3064, t3070, t3114, t3123, t3134, t4579, t4641, t4644, t4652);
        let (t14526, t14527, t14529, t14532) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1574(t13953, t14004, t14050, t14074, t14120, t14170, t14233, t14523, t349, t225, t4658, t1625, t3020);
    (t14488, t14491, t14501, t14506, t14507, t14526, t14527, t14529, t14532)
}
