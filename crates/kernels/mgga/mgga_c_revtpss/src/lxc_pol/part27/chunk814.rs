//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 814/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk814<F: Float>(t220: F, t9817: F, t124: F, t1398: F, t3938: F, t9816: F, t1410: F, t3934: F, t9757: F, t9762: F, t9766: F, t9771: F, t9776: F, t9780: F, t9786: F, t9791: F, t9796: F, t9799: F, t9804: F, t9807: F, t9812: F) -> (F, F) {
    let t9818 = t9817 * t220;
    let t9819 = t124 * t1398;
    let t9821 = t9818 * t9819 * t3938;
    let t9822 = t9816 * t9821;
    let t9824 = -F::new(0.85748036236139473944e-3) * t1410 * t9757 - F::new(0.15246000842785598468e-3) * t9762 + F::new(0.16262400898971305032e-2) * t9766 + F::new(0.21437009059034868486e-4) * t9771 - F::new(0.22866142996303859718e-3) * t9776 - F::new(0.68026775414003982663e-1) * t9780 - t9786 - t9791 - F::new(0.13553694749236397037e-4) * t9796 - F::new(0.5421477899694558815e-4) * t9799 + t9804 + F::new(0.25724410870841842183e-2) * t3934 * t9807 + F::new(0.25724410870841842183e-2) * t3934 * t9812 + F::new(0.30492001685571196935e-3) * t9822;
    (t9821, t9824)
}
