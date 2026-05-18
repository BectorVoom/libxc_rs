//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 983/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk983<F: Float>(t1872: F, t5394: F, t1291: F, t6879: F, t19847: F, t19850: F, t19852: F, t19854: F, t19858: F, t19860: F, t19863: F, t19866: F, t19868: F, t19871: F, t19873: F, t19875: F, t19877: F, t19880: F, t19883: F, t19886: F, t19888: F, t19892: F) -> (F, F, F) {
    let t20724 = t1872 * t5394;
    let t20728 = t6879 * t1291;
    let t20749 = -F::new(0.89930555555555555553e-2) * t19847 + F::new(0.26979166666666666666e-1) * t19850 + F::new(0.53958333333333333333e-1) * t19852 + F::new(0.33333333333333333333e0) * t19854 - F::new(0.53958333333333333332e-1) * t19858 - F::new(0.125e0) * t19860 + F::new(0.71944444444444444443e-1) * t19863 - F::new(0.20234375e-1) * t19866 + F::new(0.625e-1) * t19868 - F::new(0.625e-1) * t19871 - F::new(0.125e0) * t19873 + F::new(0.5e0) * t19875 + F::new(0.26979166666666666666e-1) * t19877 - F::new(0.20833333333333333333e-1) * t19880 + F::new(0.60703125e-1) * t19883 + F::new(0.10791666666666666667e0) * t19886 - F::new(0.25e0) * t19888 + F::new(0.41666666666666666667e-1) * t19892;
    (t20724, t20728, t20749)
}
