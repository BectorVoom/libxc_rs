//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta285 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk982;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk983;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk984;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk985;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta285(t1495: f64, t210: f64, t5544: f64, t10026: f64, t10029: f64, t13368: f64, t16942: f64, t16954: f64, t16988: f64, t16990: f64, t16993: f64, t16995: f64, t17000: f64, t2571: f64, t13087: f64, t13182: f64, t13234: f64, t16848: f64, t16877: f64, t16879: f64, t20882: f64, t20887: f64, t20891: f64, t20896: f64, t20958: f64, t20998: f64, t2643: f64, t843: f64, t235: f64, t20986: f64, t4282: f64, t4295: f64, t5612: f64, t1499: f64, t1523: f64, t1525: f64, t16673: f64, t20806: f64, t20854: f64, t20858: f64, t20862: f64, t20867: f64, t20871: f64, t20873: f64, t20876: f64, t20937: f64, t226: f64, t255: f64, t4166: f64, t4281: f64, t4291: f64, t5575: f64, t5645: f64, t5648: f64, t5651: f64, t5653: f64, t5655: f64, t812: f64, t858: f64, t20936: f64, t252: f64, t1492: f64, t5631: f64, t1527: f64, t5636: f64, t10110: f64, t5657: f64, t2718: f64, t1519: f64, t5558: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21008, t21011) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk982(t1495, t210, t5544, t10026, t10029, t13368, t16942, t16954, t16988, t16990, t16993, t16995, t17000, t2571);
        let t21013 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk983(t13087, t13182, t13234, t16848, t16877, t16879, t20882, t20887, t20891, t20896, t20958, t20998, t21011, t2643, t843);
        let (t21014, t21025, t21028, t21033) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk984(t21013, t235, t20986, t4282, t4295, t5612, t1499, t1523, t1525, t16673, t20806, t20854, t20858, t20862, t20867, t20871, t20873, t20876, t20937, t226, t255, t4166, t4281, t4291, t5575, t5645, t5648, t5651, t5653, t5655, t812);
        let (t21034, t21036, t21038, t21050, t21054, t21061) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk985(t21033, t858, t20936, t252, t1492, t5631, t1527, t5636, t10110, t5657, t2718, t1519, t5558);
    (t21008, t21013, t21014, t21025, t21028, t21033, t21034, t21036, t21038, t21050, t21054, t21061)
}
