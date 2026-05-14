//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 710/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk710<F: Float>(t3936: F, t3938: F, t9805: F, t3889: F, t543: F, t3937: F, t1386: F, t2482: F, t814: F, t136: F, t1412: F, t220: F, t124: F, t1398: F, t1410: F, t3934: F, t9757: F, t9762: F, t9766: F, t9771: F, t9776: F, t9780: F, t9786: F, t9791: F, t9796: F, t9799: F, t9804: F) -> (F, F, F, F) {
    let t9807 = t3936 * t9805 * t3938;
    let t9810 = t543 * t3889;
    let t9812 = t3936 * t3937 * t9810;
    let t9816 = t2482 * t1386 * t814;
    let t9817 = t1412 * t136;
    let t9818 = t9817 * t220;
    let t9819 = t124 * t1398;
    let t9821 = t9818 * t9819 * t3938;
    let t9822 = t9816 * t9821;
    let t9824 = -0.85748036236139473944e-3 * t1410 * t9757 - 0.15246000842785598468e-3 * t9762 + 0.16262400898971305032e-2 * t9766 + 0.21437009059034868486e-4 * t9771 - 0.22866142996303859718e-3 * t9776 - 0.68026775414003982663e-1 * t9780 - t9786 - t9791 - 0.13553694749236397037e-4 * t9796 - 0.5421477899694558815e-4 * t9799 + t9804 + 0.25724410870841842183e-2 * t3934 * t9807 + 0.25724410870841842183e-2 * t3934 * t9812 + 0.30492001685571196935e-3 * t9822;
    (t9807, t9812, t9821, t9824)
}
