//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta630 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1973;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1974;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1975;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1976;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta630(t86950: f64, t86955: f64, t86967: f64, t225: f64, t26708: f64, t86991: f64, t13065: f64, t2054: f64, t24325: f64, t24330: f64, t26679: f64, t2718: f64, t4147: f64, t4268: f64, t47609: f64, t7092: f64, t82108: f64, t82115: f64, t82120: f64, t85060: f64, t855: f64, t865: f64, t86997: f64, t87028: f64, t87066: f64, t87068: f64, t87080: f64, t87100: f64, t81571: f64, t81575: f64, t81592: f64, t87055: f64, t87059: f64, t87076: f64, t87078: f64, t87084: f64, t87092: f64, t87097: f64, t87104: f64, t87109: f64, t87114: f64, t87140: f64, t87153: f64, t87155: f64, t2627: f64, t7823: f64, t24273: f64, t2633: f64, t26654: f64, t26661: f64, t2679: f64, t4166: f64, t7837: f64, t808: f64, t812: f64, t81595: f64, t81600: f64, t81602: f64, t84851: f64, t87117: f64, t87124: f64, t87133: f64, t87150: f64, t87159: f64, t9612: f64, t87165: f64, t87177: f64, t26653: f64, t814: f64, t87520: f64, t1509: f64, t7084: f64, t87522: f64, t13171: f64, t1510: f64, t24256: f64, t2617: f64, t26598: f64, t26662: f64, t4291: f64, t7101: f64, t81615: f64, t81617: f64, t829: f64, t84945: f64, t87171: f64, t87174: f64, t87517: f64, t87527: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t92431, t92432, t92434, t92439, t92464) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1973(t86950, t86955, t86967, t225, t26708, t86991, t13065, t2054, t24325, t24330, t26679, t2718, t4147, t4268, t47609, t7092, t82108, t82115, t82120, t85060, t855, t865, t86997);
        let (t92486, t92506) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1974(t87028, t87066, t87068, t87080, t87100, t81571, t81575, t81592, t87055, t87059, t87076, t87078, t87084, t87092, t87097, t87104, t87109, t87114);
        let t92528 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1975(t87140, t87153, t87155, t2627, t7823, t24273, t2633, t26654, t26661, t2679, t4166, t7837, t808, t812, t81595, t81600, t81602, t84851, t87117, t87124, t87133, t87150, t87159, t9612);
        let (t92552, t92558) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1976(t87165, t87177, t26653, t814, t87520, t1509, t7084, t87522, t13171, t1510, t24256, t2617, t26598, t26662, t4166, t4291, t7101, t812, t81615, t81617, t829, t84945, t87171, t87174, t87517, t87527);
    (t92431, t92432, t92434, t92439, t92464, t92486, t92506, t92528, t92552, t92558)
}
