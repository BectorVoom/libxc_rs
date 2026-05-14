//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1208/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1208<F: Float>(t34963: F, t34984: F, t35016: F, t35035: F, t2748: F, t28036: F, t32523: F, t34836: F, t34837: F, t34838: F, t34840: F, t34842: F, t34845: F, t34889: F, t555: F, t6604: F, t8436: F, t9891: F) -> (F, F) {
    let t35037 = t34963 + t34984 + t35016 + t35035;
    let t35042 = -t2748 * t28036 + 2.0 * t32523 * t8436 + t35037 * t555 - 2.0 * t6604 * t9891 - t34836 + t34837 + t34838 - t34840 + t34842 + t34845 + t34889;
    (t35037, t35042)
}
