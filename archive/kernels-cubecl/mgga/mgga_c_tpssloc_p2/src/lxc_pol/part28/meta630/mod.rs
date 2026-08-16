//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta630 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1973;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1974;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1975;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1976;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta630<F: Float>(t86950: F, t86955: F, t86967: F, t225: F, t26708: F, t86991: F, t13065: F, t2054: F, t24325: F, t24330: F, t26679: F, t2718: F, t4147: F, t4268: F, t47609: F, t7092: F, t82108: F, t82115: F, t82120: F, t85060: F, t855: F, t865: F, t86997: F, t87028: F, t87066: F, t87068: F, t87080: F, t87100: F, t81571: F, t81575: F, t81592: F, t87055: F, t87059: F, t87076: F, t87078: F, t87084: F, t87092: F, t87097: F, t87104: F, t87109: F, t87114: F, t87140: F, t87153: F, t87155: F, t2627: F, t7823: F, t24273: F, t2633: F, t26654: F, t26661: F, t2679: F, t4166: F, t7837: F, t808: F, t812: F, t81595: F, t81600: F, t81602: F, t84851: F, t87117: F, t87124: F, t87133: F, t87150: F, t87159: F, t9612: F, t87165: F, t87177: F, t26653: F, t814: F, t87520: F, t1509: F, t7084: F, t87522: F, t13171: F, t1510: F, t24256: F, t2617: F, t26598: F, t26662: F, t4291: F, t7101: F, t81615: F, t81617: F, t829: F, t84945: F, t87171: F, t87174: F, t87517: F, t87527: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t92431, t92432, t92434, t92439, t92464) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1973::<F>(t86950, t86955, t86967, t225, t26708, t86991, t13065, t2054, t24325, t24330, t26679, t2718, t4147, t4268, t47609, t7092, t82108, t82115, t82120, t85060, t855, t865, t86997);
        let (t92486, t92506) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1974::<F>(t87028, t87066, t87068, t87080, t87100, t81571, t81575, t81592, t87055, t87059, t87076, t87078, t87084, t87092, t87097, t87104, t87109, t87114);
        let t92528 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1975::<F>(t87140, t87153, t87155, t2627, t7823, t24273, t2633, t26654, t26661, t2679, t4166, t7837, t808, t812, t81595, t81600, t81602, t84851, t87117, t87124, t87133, t87150, t87159, t9612);
        let (t92552, t92558) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1976::<F>(t87165, t87177, t26653, t814, t87520, t1509, t7084, t87522, t13171, t1510, t24256, t2617, t26598, t26662, t4166, t4291, t7101, t812, t81615, t81617, t829, t84945, t87171, t87174, t87517, t87527);
    (t92431, t92432, t92434, t92439, t92464, t92486, t92506, t92528, t92552, t92558)
}
