//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 989/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk989<F: Float>(t1936: F, t34251: F, t7359: F, t7741: F, t2055: F, t34258: F, t93: F, t7983: F, t8692: F, t1518: F, t28030: F, t32389: F, t33602: F, t33644: F, t33646: F, t34188: F, t34308: F, t6985: F, t8564: F) -> (F, F) {
    let t34310 = 2.0 * t34251 * t1936;
    let t34312 = 2.0 * t7359 * t7741;
    let t34320 = 2.0 * t34258 * t2055;
    let t34321 = t93 * t7741;
    let t34323 = 2.0 * t34321 * t2055;
    let t34325 = 2.0 * t8692 * t7983;
    let t34326 = 2.0 * t1518 * t32389 + 2.0 * t2055 * t28030 + 2.0 * t2055 * t33602 + 2.0 * t6985 * t7983 + t33644 + t33646 + t34188 + t34308 + t34310 + t34312 + t34320 + t34323 + t34325 + t8564;
    (t34321, t34326)
}
