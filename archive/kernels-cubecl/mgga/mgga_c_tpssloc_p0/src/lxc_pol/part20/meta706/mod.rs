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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2690;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2691;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2692;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2693;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2694;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2695;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2696;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2697;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta706<F: Float>(t12283: F, t16244: F, t1307: F, t3791: F, t12279: F, t12419: F, t12420: F, t12422: F, t12426: F, t12429: F, t16233: F, t16242: F, t16305: F, t16366: F, t16394: F, t19876: F, t3793: F, t3803: F, t39975: F, t40329: F, t5246: F, t5248: F, t5249: F, t5259: F, t5303: F, t54014: F, t54739: F, t54744: F, t54745: F, t54750: F, t554: F, t559: F, t3862: F, t5231: F, t16356: F, t3726: F, t12328: F, t1815: F, t16397: F, t3777: F, t5252: F, t1336: F, t2691: F, t3788: F, t119: F, t12407: F, t1315: F, t16248: F, t16265: F, t16364: F, t16383: F, t210: F, t3805: F, t3851: F, t3856: F, t40443: F, t40449: F, t53856: F, t53905: F, t53943: F, t53978: F, t54026: F, t54058: F, t54100: F, t54137: F, t54183: F, t54215: F, t54245: F, t54277: F, t54552: F, t54584: F, t54625: F, t16028: F, t225: F, t12022: F, t12437: F, t12438: F, t1375: F, t1386: F, t16437: F, t16460: F, t16471: F, t16475: F, t1842: F, t1843: F, t3758: F, t3887: F, t3912: F, t39913: F, t39916: F, t39919: F, t40591: F, t5215: F, t53866: F, t539: F, t568: F, t1372: F, t5286: F, t1824: F, t3879: F, t12240: F, t1351: F, t16205: F, t562: F, t12168: F, t1352: F, t16036: F, t16040: F, t16041: F, t16047: F, t16048: F, t16055: F, t26409: F, t3773: F, t5333: F, t5334: F, t5335: F, t5336: F, t5343: F, t5344: F, t5345: F, t3787: F, t5318: F, t1834: F, t3850: F, t12248: F, t12172: F, t12251: F, t12267: F, t16033: F, t16060: F, t16125: F, t16127: F, t3898: F, t40335: F, t5234: F, t5250: F, t5339: F, t5341: F, t40041: F, t544: F, t68: F, t1332: F, t16046: F, t12169: F, t12178: F, t12259: F, t12273: F, t12435: F, t16068: F, t16132: F, t16433: F, t1814: F, t1838: F, t19810: F, t40118: F, t5287: F, t5348: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t54776 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2690::<F>(t12283, t16244, t1307, t3791, t12279, t12419, t12420, t12422, t12426, t12429, t16233, t16242, t16305, t16366, t16394, t19876, t3793, t3803, t39975, t40329, t5246, t5248, t5249, t5259, t5303, t54014, t54739, t54744, t54745, t54750, t554, t559);
        let (t54786, t54787, t54793, t54801, t54811) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2691::<F>(t3862, t5231, t16356, t3726, t12328, t1815, t16397, t3777, t5252, t1336, t2691, t3788);
        let t54813 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2692::<F>(t54811, t119, t12407, t12429, t1315, t16242, t16248, t16265, t16364, t16383, t210, t3803, t3805, t3851, t3856, t40443, t40449, t5248, t53856, t54786, t54787, t54793, t54801);
        let t54817 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2693::<F>(t53905, t53943, t53978, t54026, t54058, t54100, t54137, t54183, t54215, t54245, t54277, t54552, t54584, t54625, t54776, t54813);
        let t54832 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2694::<F>(t16028, t225, t12022, t12437, t12438, t1375, t1386, t16437, t16460, t16471, t16475, t1842, t1843, t3758, t3887, t3912, t39913, t39916, t39919, t40591, t5215, t53866, t539, t54817, t568);
        let (t54840, t54854, t54858, t54883, t54900) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2695::<F>(t1372, t5286, t1824, t3879, t12240, t1351, t16205, t562, t12168, t1352, t16036, t16040, t16041, t16047, t16048, t16055, t26409, t3773, t3793, t3851, t3856, t5333, t5334, t5335, t5336, t5343, t5344, t5345);
        let (t54905, t54918, t54959) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2696::<F>(t3787, t5318, t1834, t3850, t12248, t12172, t12251, t12267, t1336, t1351, t1352, t16033, t16036, t16047, t16060, t16125, t16127, t3777, t3856, t3898, t40335, t5234, t5250, t5334, t5335, t5339, t5341, t5344, t54854, t54883);
        let (t54963, t54976, t55012) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2697::<F>(t40041, t544, t68, t1332, t16046, t1352, t3850, t12169, t12178, t12259, t12273, t12435, t1336, t16033, t16068, t16132, t16433, t1814, t1838, t19810, t3777, t3851, t3856, t40118, t5234, t5287, t5335, t5344, t5348);
    (t54817, t54832, t54840, t54854, t54858, t54900, t54905, t54918, t54959, t54963, t54976, t55012)
}
