//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1038/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1038<F: Float>(t92988: F, t92995: F, t92997: F, t92999: F, t93007: F, t93012: F, t92979: F, t92982: F, t92984: F, t92991: F, t93001: F, t93004: F, t93010: F, t93016: F, t93020: F, t93022: F, t93026: F, t93028: F, t93031: F, t93035: F, t93037: F, t93039: F, t93041: F, t93043: F, t93045: F, t93049: F, t93051: F, t93055: F) -> (F, F) {
    let t95671 = 0.3252886739816735289e-3 * t92988;
    let t95673 = 455.0 / 648.0 * t92995;
    let t95674 = 0.15117061203111996147e0 * t92997;
    let t95675 = 0.51384669507166276316e-2 * t92999;
    let t95678 = 0.80328230880474379779e-6 * t93007;
    let t95680 = 0.45178982497454656792e-6 * t93012;
    let t95682 = -7.0 / 8.0 * t92979 - t92982 / 2.0 + 3.0 / 8.0 * t92984 - t95671 + 0.12196800674228478774e-3 * t92991 - t95673 - t95674 + t95675 - 0.3658582879408617555e-2 * t93001 + 0.34299214494455789577e-3 * t93004 + t95678 - 0.17149607247227894789e-2 * t93010 - t95680 - 0.54214778996945588151e-4 * t93016;
    let t95684 = 0.28900264064772933812e-2 * t93020;
    let t95698 = -t95684 - 0.20579528696673473747e-1 * t93022 + 0.30492001685571196935e-3 * t93026 + 0.12004725073059526352e-1 * t93028 - 0.68598428988911579154e-3 * t93031 + 0.16262400898971305032e-2 * t93035 + 0.51448821741683684367e-1 * t93037 + 0.51448821741683684367e-2 * t93039 - 0.85748036236139473944e-3 * t93041 - 0.15246000842785598468e-3 * t93043 + 0.12004725073059526352e-1 * t93045 - 0.68026775414003982662e-1 * t93049 - 0.85748036236139473944e-3 * t93051 - 0.24009450146119052704e-1 * t93055;
    (t95682, t95698)
}
