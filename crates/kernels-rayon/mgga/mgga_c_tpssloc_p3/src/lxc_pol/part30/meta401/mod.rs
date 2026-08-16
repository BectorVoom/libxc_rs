//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta401 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1526;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1527;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1528;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1529;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1530;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta401(t3131: f64, t4649: f64, t4593: f64, t4582: f64, t16558: f64, t998: f64, t974: f64, t13835: f64, t4531: f64, t13769: f64, t13839: f64, t1539: f64, t6733: f64, t4540: f64, t7577: f64, t4546: f64, t343: f64, t5842: f64, t984: f64, t2970: f64, t5824: f64, t973: f64, t10226: f64, t13782: f64, t13787: f64, t13790: f64, t13825: f64, t2960: f64, t2986: f64, t5825: f64, t5828: f64, t978: f64, t977: f64, t5836: f64, t10231: f64, t5817: f64, t13861: f64, t17178: f64, t4510: f64, t2989: f64, t5398: f64, t2988: f64, t10186: f64, t13830: f64, t13850: f64, t5818: f64, t5821: f64, t5829: f64, t2987: f64, t2990: f64, t13847: f64, t4514: f64, t17167: f64, t4518: f64, t17171: f64, t10254: f64, t5392: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17732, t17734, t17738, t17742, t17745, t17748) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1526(t3131, t4649, t4593, t4582, t16558, t998, t974, t13835, t4531, t13769, t13839, t1539, t6733);
        let t17766 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1527(t17748, t4531, t4540, t7577, t4546, t343, t5842, t984, t2970, t5824, t973, t10226, t13782, t13787, t13790, t13825, t17742, t17745, t2960, t2986, t5825);
        let (t17770, t17773, t17778, t17783) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1528(t2970, t5828, t973, t16558, t978, t977, t343, t5836, t984, t4546, t10231, t5817);
        let t17798 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1529(t17783, t973, t13861, t4531, t17178, t4510, t2989, t5398, t2988, t10186, t13830, t13850, t17770, t17773, t17778, t2960, t2986, t5818, t5821, t5829);
        let (t17801, t17805, t17809, t17811, t17814, t17817) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1530(t2987, t5836, t2990, t5842, t13847, t4514, t2986, t17167, t4518, t17171, t10254, t5392);
    (t17732, t17734, t17738, t17766, t17798, t17801, t17805, t17809, t17811, t17814, t17817)
}
