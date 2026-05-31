//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1049/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1049<F: Float>(t1936: F, t33602: F, t6985: F, t7741: F, t1501: F, t8453: F, t8692: F, t4248: F, t8460: F, t7889: F, t7742: F, t8634: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t33635 = t33602 * t1936;
    let t33637 = t6985 * t7741;
    let t33639 = t1501 * t8453;
    let t33640 = F::cast_from(2.0_f64) * t33639;
    let t33642 = F::cast_from(4.0_f64) * t8692 * t7741;
    let t33643 = t4248 * t8460;
    let t33644 = F::cast_from(2.0_f64) * t33643;
    let t33645 = t7889 * t8460;
    let t33646 = F::cast_from(2.0_f64) * t33645;
    let t33650 = F::cast_from(4.0_f64) * t8634 * t7742;
    (t33635, t33637, t33639, t33640, t33642, t33643, t33644, t33645, t33646, t33650)
}
