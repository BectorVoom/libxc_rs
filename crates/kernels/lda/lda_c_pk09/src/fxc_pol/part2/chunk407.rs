//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 407/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk407<F: Float>(t1684: F, t1735: F, t1732: F, t1738: F, t1762: F, t1769: F, t2061: F, t2063: F, t467: F, t452: F, t1748: F, t2026: F, t2027: F, t2029: F, t2030: F, t2032: F, t2037: F, t2044: F, t2047: F, t2053: F, t2058: F, t2060: F, t453: F, t455: F, t463: F, t472: F) -> (F, F, F, F, F, F) {
    let t2065 = F::new(0.3056501876701794) * t1684;
    let t2067 = F::new(0.1018833958900598) * t1735;
    let t2069 = t2061 - F::new(1.5323028051206833) * t1762 + t2063 + F::new(1.5323028051206833) * t1769 + t2065 - F::new(0.3056501876701794) * t1732 + t2067 + F::new(0.3056501876701794) * t1738;
    let t2070 = t467 * t2069;
    let t2071 = t2070 * t452;
    let t2074 = -t2026 - t2027 - F::new(0.10237773105191754) * t1738 - t2029 - t2030 + t463 * t2032 / F::new(6.0) - t2037 * t1748 / F::new(6.0) + t2044 - t2047 - t472 * t2032 / F::new(6.0) + t453 * t2032 / F::new(6.0) - t2053 * t1748 / F::new(6.0) + t2058 + t2060 + t2071 * t455 / F::new(6.0);
    (t2065, t2067, t2069, t2070, t2071, t2074)
}
