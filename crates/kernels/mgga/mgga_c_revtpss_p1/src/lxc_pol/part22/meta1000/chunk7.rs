//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3405/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3405<F: Float>(t324: F, t63847: F, t63861: F, t63875: F, t63889: F, t300: F, t11506: F, t15542: F, t6205: F, t981: F, t15566: F, t19153: F, t3329: F, t5023: F, t63673: F, t63676: F, t63679: F, t63681: F, t63683: F, t63685: F, t63820: F, t63826: F, t63827: F, t63833: F, t63835: F) -> (F, F, F, F) {
    let t63892 = (t63847 + t63861 + t63875 + t63889) * t324;
    let t63894 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t63892;
    let t63898 = F::cast_from(0.10254018858216406658e4_f64) * t981 * t11506 * t6205 * t15542;
    let t63899 = F::cast_from(8.0_f64) * t15566 * t5023 * t63827 - t19153 * t3329 * t5023 - t63673 - t63676 - t63679 + t63681 - t63683 + t63685 - t63820 + t63826 - t63833 - t63835 + t63894 - t63898;
    (t63892, t63894, t63898, t63899)
}
