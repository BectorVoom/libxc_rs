//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1371/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1371<F: Float>(t10297: F, t595: F, t637: F, t10300: F, t10288: F, t10269: F, t1655: F, t21396: F, t21401: F, t21404: F, t216: F, t26481: F, t26488: F, t26490: F, t26493: F, t32194: F, t32195: F, t598: F) -> (F,) {
    let t33499 = t595 * t10297 * t637;
    let t33502 = t595 * t10300 * t637;
    let t33505 = t595 * t10288 * t637;
    let t33513 = -0.50808839199999999999e-2 * t21396 - t21401 - 0.21973736767207854065e-2 * t32195 * t216 + t21404 + 0.15431256e1 * t26481 + 0.48618743904e1 * t26488 - 0.34675007859127131175e2 * t26490 - 0.20010214504933333333e-2 * t33499 - 0.60030643514799999999e-2 * t33502 - 0.60030643514799999999e-2 * t33505 + 0.17337503929563565587e2 * t26493 - 0.675260332e-1 * t595 * t32194 * t598 - 0.675260332e-1 * t10269 * t1655;
    (t33513,)
}
