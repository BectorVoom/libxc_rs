//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1147/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1147<F: Float>(t22843: F, t22844: F, t22847: F, t22850: F, t22853: F, t22857: F, t22859: F, t22860: F, t22861: F, t22862: F, t22863: F, t22868: F, t23321: F, t22872: F, t22875: F, t22880: F, t22885: F, t22889: F, t22890: F, t22892: F, t22894: F, t22898: F, t22900: F, t22902: F, t22904: F, t22906: F) -> (F, F) {
    let t23323 = t22843 - t22844 + t22847 - t22850 + t22853 + t22857 - t22859 + t22860 + t22861 - t22862 + t22863 + 0.10821041362364843 * t23321 + t22868;
    let t23324 = -t22872 + t22875 - t22880 - t22885 + t22889 - t22890 + t22892 + t22894 + t22898 - t22900 + t22902 - t22904 - t22906;
    (t23323, t23324)
}
