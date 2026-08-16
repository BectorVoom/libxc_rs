//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 401/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk401<F: Float>(t1832: F, t436: F, t10: F, t127: F, t1655: F, t1663: F, t1674: F, t1676: F, t1689: F, t1692: F, t1695: F, t1813: F, t1816: F, t1819: F, t1835: F, t1838: F, t1840: F, t1845: F, t1850: F, t1852: F, t411: F, t426: F) -> (F, F) {
    let t1856 = t436 * t1832;
    let t1859 = -t1655 + t1813 + t1663 + t1816 + t1819 - t1835 + t1674 + t1676 / F::cast_from(6.0_f64) + t1838 / F::cast_from(6.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t426 * t10 * t1840 - t426 * t1845 / F::cast_from(2.0_f64) + t1689 + F::cast_from(0.73452_f64) * t1692 + t1695 + F::cast_from(0.73452_f64) * t1850 + F::cast_from(5.87616_f64) * t127 * t1852 * t411 - F::cast_from(1.46904_f64) * t127 * t1856;
    (t1856, t1859)
}
