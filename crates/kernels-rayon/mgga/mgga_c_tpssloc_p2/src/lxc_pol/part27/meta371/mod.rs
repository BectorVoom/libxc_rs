//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta371 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1527;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1528;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1529;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta371(t13748: f64, t973: f64, t1611: f64, t3088: f64, t1036: f64, t4617: f64, t1023: f64, t4347: f64, t3071: f64, t10422: f64, t4574: f64, t3070: f64, t1597: f64, t4509: f64, t10237: f64, t10189: f64, t344: f64, t4343: f64, t2986: f64, t134: f64, t2978: f64, t4338: f64, t10190: f64, t4514: f64, t13528: f64, t4510: f64, t13532: f64, t10213: f64, t60: f64, t13537: f64, t10186: f64, t10192: f64, t10226: f64, t10229: f64, t4511: f64, t4515: f64, t4519: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13750, t13751, t13758, t13762, t13765, t13767) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1527(t13748, t973, t1611, t3088, t1036, t4617, t1023, t4347, t3071, t10422, t4574, t3070);
        let (t13770, t13782, t13783, t13787, t13788) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1528(t1597, t4509, t10237, t10189, t344, t4343, t2986, t134, t2978, t4338, t10190, t4514);
        let (t13797, t13804) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1529(t13788, t2986, t13528, t4510, t13532, t10213, t60, t344, t13537, t10186, t10192, t10226, t10229, t13770, t13782, t13787, t4511, t4515, t4519);
    (t13750, t13751, t13758, t13762, t13765, t13767, t13783, t13797, t13804)
}
