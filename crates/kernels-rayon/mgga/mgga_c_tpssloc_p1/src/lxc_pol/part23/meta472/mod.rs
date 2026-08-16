//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta472 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1408;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1409;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1410;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1411;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1412;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta472(t123: f64, t3240: f64, t77965: f64, t63332: f64, t63334: f64, t63888: f64, t63893: f64, t63911: f64, t71142: f64, t71144: f64, t71146: f64, t71152: f64, t71154: f64, t71156: f64, t71408: f64, t78002: f64, t5999: f64, t3270: f64, t5992: f64, t43889: f64, t1409: f64, t71137: f64, t18205: f64, t5398: f64, t11145: f64, t18210: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t78005 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1408(t123, t3240, t77965);
        let t78019 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1409(t63332, t63334, t63888, t63893, t63911, t71142, t71144, t71146, t71152, t71154, t71156, t71408, t78002, t78005);
        let (t78025, t78026, t78028, t78029, t78031, t78033) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1410(t5999, t3270, t5992, t43889, t1409, t71137, t123, t3240);
        let (t78035, t78037) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1411(t18205, t5398, t11145, t123);
        let (t78039, t78041) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1412(t18210, t5398, t123, t3240);
    (t78005, t78019, t78025, t78026, t78028, t78029, t78031, t78033, t78035, t78037, t78039, t78041)
}
