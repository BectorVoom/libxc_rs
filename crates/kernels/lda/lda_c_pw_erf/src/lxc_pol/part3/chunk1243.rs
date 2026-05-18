//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1243/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1243<F: Float>(t3338: F, t770: F, t1710: F, t1859: F, t14581: F, t426: F, t1849: F, t1953: F, t127: F, t14549: F, t14552: F, t14555: F, t14558: F, t14561: F, t14562: F, t1568: F, t1664: F, t1697: F, t1832: F, t1852: F, t3251: F, t3296: F, t411: F, t5548: F, t5578: F, t8884: F) -> (F, F, F, F, F) {
    let t14724 = t770 * t3338;
    let t14729 = t1859 * t1710;
    let t14732 = t426 * t14581;
    let t14734 = t1849 * t1953;
    let t14756 = F::new(17.62848) * t127 * t5578 * t1568 + F::new(5.87616) * t127 * t1852 * t3251 - F::new(88.1424) * t127 * t3296 * t1832 * t1664 + F::new(17.62848) * t127 * t1697 * t5548 * t411 + t14549 + t14552 - t14555 + t14558 + t14561 - t14562 - F::new(3.0) / F::new(2.0) * t8884;
    (t14724, t14729, t14732, t14734, t14756)
}
