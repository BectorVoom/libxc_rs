//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta706 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2690;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2691;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2692;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2693;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2694;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2695;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2696;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2697;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta706(t12283: f64, t16244: f64, t1307: f64, t3791: f64, t12279: f64, t12419: f64, t12420: f64, t12422: f64, t12426: f64, t12429: f64, t16233: f64, t16242: f64, t16305: f64, t16366: f64, t16394: f64, t19876: f64, t3793: f64, t3803: f64, t39975: f64, t40329: f64, t5246: f64, t5248: f64, t5249: f64, t5259: f64, t5303: f64, t54014: f64, t54739: f64, t54744: f64, t54745: f64, t54750: f64, t554: f64, t559: f64, t3862: f64, t5231: f64, t16356: f64, t3726: f64, t12328: f64, t1815: f64, t16397: f64, t3777: f64, t5252: f64, t1336: f64, t2691: f64, t3788: f64, t119: f64, t12407: f64, t1315: f64, t16248: f64, t16265: f64, t16364: f64, t16383: f64, t210: f64, t3805: f64, t3851: f64, t3856: f64, t40443: f64, t40449: f64, t53856: f64, t53905: f64, t53943: f64, t53978: f64, t54026: f64, t54058: f64, t54100: f64, t54137: f64, t54183: f64, t54215: f64, t54245: f64, t54277: f64, t54552: f64, t54584: f64, t54625: f64, t16028: f64, t225: f64, t12022: f64, t12437: f64, t12438: f64, t1375: f64, t1386: f64, t16437: f64, t16460: f64, t16471: f64, t16475: f64, t1842: f64, t1843: f64, t3758: f64, t3887: f64, t3912: f64, t39913: f64, t39916: f64, t39919: f64, t40591: f64, t5215: f64, t53866: f64, t539: f64, t568: f64, t1372: f64, t5286: f64, t1824: f64, t3879: f64, t12240: f64, t1351: f64, t16205: f64, t562: f64, t12168: f64, t1352: f64, t16036: f64, t16040: f64, t16041: f64, t16047: f64, t16048: f64, t16055: f64, t26409: f64, t3773: f64, t5333: f64, t5334: f64, t5335: f64, t5336: f64, t5343: f64, t5344: f64, t5345: f64, t3787: f64, t5318: f64, t1834: f64, t3850: f64, t12248: f64, t12172: f64, t12251: f64, t12267: f64, t16033: f64, t16060: f64, t16125: f64, t16127: f64, t3898: f64, t40335: f64, t5234: f64, t5250: f64, t5339: f64, t5341: f64, t40041: f64, t544: f64, t68: f64, t1332: f64, t16046: f64, t12169: f64, t12178: f64, t12259: f64, t12273: f64, t12435: f64, t16068: f64, t16132: f64, t16433: f64, t1814: f64, t1838: f64, t19810: f64, t40118: f64, t5287: f64, t5348: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t54776 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2690(t12283, t16244, t1307, t3791, t12279, t12419, t12420, t12422, t12426, t12429, t16233, t16242, t16305, t16366, t16394, t19876, t3793, t3803, t39975, t40329, t5246, t5248, t5249, t5259, t5303, t54014, t54739, t54744, t54745, t54750, t554, t559);
        let (t54786, t54787, t54793, t54801, t54811) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2691(t3862, t5231, t16356, t3726, t12328, t1815, t16397, t3777, t5252, t1336, t2691, t3788);
        let t54813 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2692(t54811, t119, t12407, t12429, t1315, t16242, t16248, t16265, t16364, t16383, t210, t3803, t3805, t3851, t3856, t40443, t40449, t5248, t53856, t54786, t54787, t54793, t54801);
        let t54817 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2693(t53905, t53943, t53978, t54026, t54058, t54100, t54137, t54183, t54215, t54245, t54277, t54552, t54584, t54625, t54776, t54813);
        let t54832 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2694(t16028, t225, t12022, t12437, t12438, t1375, t1386, t16437, t16460, t16471, t16475, t1842, t1843, t3758, t3887, t3912, t39913, t39916, t39919, t40591, t5215, t53866, t539, t54817, t568);
        let (t54840, t54854, t54858, t54883, t54900) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2695(t1372, t5286, t1824, t3879, t12240, t1351, t16205, t562, t12168, t1352, t16036, t16040, t16041, t16047, t16048, t16055, t26409, t3773, t3793, t3851, t3856, t5333, t5334, t5335, t5336, t5343, t5344, t5345);
        let (t54905, t54918, t54959) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2696(t3787, t5318, t1834, t3850, t12248, t12172, t12251, t12267, t1336, t1351, t1352, t16033, t16036, t16047, t16060, t16125, t16127, t3777, t3856, t3898, t40335, t5234, t5250, t5334, t5335, t5339, t5341, t5344, t54854, t54883);
        let (t54963, t54976, t55012) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2697(t40041, t544, t68, t1332, t16046, t1352, t3850, t12169, t12178, t12259, t12273, t12435, t1336, t16033, t16068, t16132, t16433, t1814, t1838, t19810, t3777, t3851, t3856, t40118, t5234, t5287, t5335, t5344, t5348);
    (t54817, t54832, t54840, t54854, t54858, t54900, t54905, t54918, t54959, t54963, t54976, t55012)
}
