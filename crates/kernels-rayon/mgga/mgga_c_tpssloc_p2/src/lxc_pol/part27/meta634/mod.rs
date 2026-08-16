//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta634 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2136;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2137;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2138;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2139;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta634(t22986: f64, t22996: f64, t25249: f64, t2633: f64, t81602: f64, t252: f64, t4119: f64, t6646: f64, t829: f64, t25160: f64, t814: f64, t22690: f64, t7520: f64, t81573: f64, t2627: f64, t7510: f64, t13171: f64, t1510: f64, t6657: f64, t812: f64, t81599: f64, t81600: f64, t81718: f64, t87097: f64, t87101: f64, t87104: f64, t87109: f64, t87114: f64, t87117: f64, t87119: f64, t2684: f64, t25324: f64, t6562: f64, t794: f64, t23030: f64, t25258: f64, t13384: f64, t2647: f64, t22893: f64, t23164: f64, t25306: f64, t81615: f64, t25236: f64, t13381: f64, t1888: f64, t7524: f64, t81612: f64, t81613: f64, t4240: f64, t81865: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87124, t87127, t87130, t87133, t87135, t87140) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2136(t22986, t22996, t25249, t2633, t81602, t252, t4119, t6646, t829, t25160, t814, t22690, t7520, t81573);
        let t87146 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2137(t2627, t7510, t13171, t1510, t2633, t6657, t812, t81599, t81600, t81718, t829, t87097, t87101, t87104, t87109, t87114, t87117, t87119, t87124, t87127, t87133, t87135, t87140);
        let (t87150, t87154, t87155, t87159, t87165) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2138(t22986, t25249, t2684, t6646, t25324, t6562, t794, t23030, t25258, t13384, t2647, t22893, t23164, t25306);
        let (t87166, t87167, t87171, t87174, t87177, t87183) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2139(t87165, t81615, t22986, t25236, t2647, t6646, t13381, t1888, t7524, t81612, t81613, t4240, t81865);
    (t87130, t87146, t87150, t87154, t87155, t87159, t87166, t87167, t87171, t87174, t87177, t87183)
}
