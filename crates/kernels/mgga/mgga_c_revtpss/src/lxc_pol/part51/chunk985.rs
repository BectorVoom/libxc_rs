//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 985/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk985<F: Float>(t33643: F, t7889: F, t8460: F, t1518: F, t32162: F, t33630: F, t33633: F, t33635: F, t33637: F, t33640: F, t33642: F, t8564: F) -> (F, F, F) {
    let t33644 = F::new(2.0) * t33643;
    let t33645 = t7889 * t8460;
    let t33646 = F::new(2.0) * t33645;
    let t33647 = F::new(2.0) * t1518 * t32162 + t33630 + F::new(4.0) * t33633 + F::new(4.0) * t33635 + F::new(4.0) * t33637 + t33640 + t33642 + t33644 + t33646 + t8564;
    (t33644, t33646, t33647)
}
