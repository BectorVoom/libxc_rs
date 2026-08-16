//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta436 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1675;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1676;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1677;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta436<F: Float>(t1362: F, t19815: F, t3799: F, t6417: F, t6422: F, t1307: F, t6330: F, t12351: F, t820: F, t1799: F, t5187: F, t3870: F, t1367: F, t19631: F, t16336: F, t1831: F, t12308: F, t12325: F, t12330: F, t12335: F, t1363: F, t1369: F, t16321: F, t16346: F, t16350: F, t16354: F, t3778: F, t3783: F, t5240: F, t5310: F, t5314: F, t6427: F, t6431: F, t3866: F, t19735: F, t5248: F, t5249: F, t16242: F, t3805: F, t6394: F, t120: F, t6414: F, t3807: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t19904, t19915, t19917, t19919, t19921, t19924, t19926) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1675::<F>(t1362, t19815, t3799, t6417, t6422, t1307, t6330, t12351, t820, t1799, t5187, t3870);
        let (t19930, t19939) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1676::<F>(t1367, t19631, t820, t16336, t1831, t12308, t12325, t12330, t12335, t1363, t1369, t16321, t16346, t16350, t16354, t19904, t19915, t19917, t19921, t19926, t3778, t3783, t5240, t5310, t5314, t6422, t6427, t6431);
        let (t19940, t19942, t19945, t19951, t19956, t19958) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1677::<F>(t3866, t6427, t6431, t19735, t5248, t5249, t16242, t3805, t6394, t120, t6414, t3807);
    (t19919, t19921, t19924, t19926, t19930, t19939, t19940, t19942, t19945, t19951, t19956, t19958)
}
