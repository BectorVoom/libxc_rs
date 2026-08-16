//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta233 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk940;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk941;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk942;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk943;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk944;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk945;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk946;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta233(t1060: f64, t11077: f64, t11023: f64, t3201: f64, t1003: f64, t10359: f64, t1058: f64, t1061: f64, t1063: f64, t11024: f64, t11028: f64, t11031: f64, t11034: f64, t11037: f64, t11040: f64, t11043: f64, t11046: f64, t11049: f64, t11051: f64, t11055: f64, t11059: f64, t11061: f64, t11065: f64, t11067: f64, t3076: f64, t3180: f64, t3186: f64, t3189: f64, t3193: f64, t3197: f64, t3200: f64, t3202: f64, t3204: f64, t353: f64, t384: f64, t1055: f64, t10160: f64, t10167: f64, t10170: f64, t10182: f64, t1052: f64, t1066: f64, t11008: f64, t11010: f64, t11013: f64, t11016: f64, t11018: f64, t3026: f64, t3169: f64, t3176: f64, t3207: f64, t388: f64, t1068: f64, t3213: f64, t3215: f64, t390: f64, t10521: f64, t10528: f64, t10607: f64, t10625: f64, t10627: f64, t10635: f64, t1070: f64, t10711: f64, t10729: f64, t10733: f64, t10849: f64, t10851: f64, t193: f64, t336: f64, t10622: f64, t10649: f64, t10652: f64, t10654: f64, t10657: f64, t10665: f64, t10699: f64, t10707: f64, t10715: f64, t10739: f64, t10819: f64, t10855: f64, t3209: f64, t3216: f64, t4700: f64, t25: f64, t265: f64, t394: f64, t10150: f64, t1074: f64, t2249: f64, t2250: f64, t2756: f64, t3220: f64, t396: f64, t40: f64, t606: f64, t607: f64, t873: f64, t9257: f64, t9258: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t300: f64, t3368: f64, t1166: f64, t1155: f64, t3377: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11078, t11081, t11084) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk940(t1060, t11077, t11023, t3201, t1003, t10359, t1058, t1061, t1063, t11024, t11028, t11031, t11034, t11037, t11040, t11043, t11046, t11049, t11051, t11055, t11059, t11061, t11065, t11067, t3076, t3180, t3186, t3189, t3193, t3197, t3200, t3202, t3204, t353, t384);
        let (t11085, t11087) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk941(t1055, t11084, t10160, t10167, t10170, t10182, t1052, t1066, t11008, t11010, t11013, t11016, t11018, t3026, t3169, t3176, t3207, t388);
        let (t11094, t11098) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk942(t1068, t3213, t3215, t390, t10521, t10528, t10607, t10625, t10627, t10635, t1070, t10711, t10729, t10733, t10849, t10851, t11087, t193, t336);
        let t11103 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk943(t10622, t10649, t10652, t10654, t10657, t10665, t1068, t10699, t10707, t10715, t10739, t10819, t10855, t3209, t3216, t4700);
        let (t11105, t11115) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk944(t25, t265, t394, t10150, t11098, t11103, t1074, t2249, t2250, t2756, t3220, t396, t40, t606, t607, t873, t9257, t9258, dens_threshold, rho0, zeta_threshold);
        let t11122 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk945(t9257);
        let (t11126, t11128, t11129) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk946(t300, t3368, t1166, t1155, t3377);
    (t11078, t11081, t11084, t11085, t11087, t11094, t11105, t11115, t11122, t11126, t11128, t11129)
}
