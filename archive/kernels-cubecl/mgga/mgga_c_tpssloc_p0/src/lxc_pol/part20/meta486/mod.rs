//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta486 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1972;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1973;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1974;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta486<F: Float>(t12199: F, t5202: F, t118: F, t5187: F, t794: F, t3739: F, t16018: F, t210: F, t214: F, t12225: F, t16095: F, t2586: F, t12236: F, t1315: F, t16083: F, t16086: F, t16090: F, t16099: F, t16101: F, t16103: F, t16106: F, t5195: F, t16080: F, t225: F, t3856: F, t5335: F, t3851: F, t5348: F, t1332: F, t1336: F, t1381: F, t16033: F, t16037: F, t16041: F, t16044: F, t16047: F, t16049: F, t16052: F, t16055: F, t16060: F, t16065: F, t16068: F, t3777: F, t3902: F, t5234: F, t5334: F, t5336: F, t5344: F, t5345: F, t5349: F, t5351: F, t564: F) -> (F, F, F, F, F, F, F, F) {
        let (t16108, t16111, t16113, t16115, t16118, t16119) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1972::<F>(t12199, t5202, t118, t5187, t794, t3739, t16018, t210, t214, t12225, t16095, t2586);
        let t16121 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1973::<F>(t12236, t1315, t16083, t16086, t16090, t16099, t16101, t16103, t16106, t16108, t16113, t16115, t16119, t5195);
        let (t16122, t16123, t16125, t16127, t16131) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1974::<F>(t16080, t16121, t225, t3856, t5335, t3851, t5348, t1332, t1336, t1381, t16033, t16037, t16041, t16044, t16047, t16049, t16052, t16055, t16060, t16065, t16068, t3777, t3902, t5234, t5334, t5336, t5344, t5345, t5349, t5351, t564);
    (t16111, t16115, t16118, t16122, t16123, t16125, t16127, t16131)
}
