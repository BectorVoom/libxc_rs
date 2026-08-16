//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta406 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1685;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1686;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1687;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1688;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1689;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1690;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta406(t5194: f64, t782: f64, t5198: f64, t213: f64, t5187: f64, t1307: f64, t221: f64, t3719: f64, t5196: f64, t3732: f64, t67: f64, t792: f64, t1799: f64, t212: f64, t686: f64, t12214: f64, t131: f64, t205: f64, t3734: f64, t3726: f64, t5206: f64, t12199: f64, t5202: f64, t118: f64, t794: f64, t3739: f64, t16018: f64, t210: f64, t214: f64, t12225: f64, t2586: f64, t12236: f64, t1315: f64, t5195: f64, t16080: f64, t225: f64, t3856: f64, t5335: f64, t3851: f64, t5348: f64, t1332: f64, t1336: f64, t1381: f64, t16033: f64, t16037: f64, t16041: f64, t16044: f64, t16047: f64, t16049: f64, t16052: f64, t16055: f64, t16060: f64, t16065: f64, t16068: f64, t3777: f64, t3902: f64, t5234: f64, t5334: f64, t5336: f64, t5344: f64, t5345: f64, t5349: f64, t5351: f64, t564: f64, t1338: f64, t5318: f64, t1352: f64, t12259: f64, t1825: f64, t3866: f64, t5310: f64, t3870: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16083, t16086, t16090, t16094) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1685(t5194, t782, t5198, t213, t5187, t1307, t221, t3719, t5196, t3732, t67, t792);
        let (t16095, t16099, t16101, t16103, t16106) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1686(t1799, t212, t1307, t686, t16094, t12214, t131, t205, t221, t3734, t5196, t3726, t5206);
        let (t16108, t16113, t16115, t16119) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1687(t12199, t5202, t118, t5187, t794, t3739, t16018, t210, t214, t12225, t16095, t2586);
        let t16121 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1688(t12236, t1315, t16083, t16086, t16090, t16099, t16101, t16103, t16106, t16108, t16113, t16115, t16119, t5195);
        let (t16122, t16123, t16125, t16131) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1689(t16080, t16121, t225, t3856, t5335, t3851, t5348, t1332, t1336, t1381, t16033, t16037, t16041, t16044, t16047, t16049, t16052, t16055, t16060, t16065, t16068, t3777, t3902, t5234, t5334, t5336, t5344, t5345, t5349, t5351, t564);
        let (t16133, t16136, t16147, t16148, t16150, t16153) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1690(t1338, t5318, t1352, t12259, t1825, t3866, t5310, t1307, t5187, t3870, t820, t1799, t3719);
    (t16122, t16123, t16125, t16131, t16133, t16136, t16147, t16148, t16150, t16153)
}
