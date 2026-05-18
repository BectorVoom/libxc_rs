//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1175/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1175<F: Float>(t93020: F, t93022: F, t93026: F, t93028: F, t93031: F, t93035: F, t93037: F, t93039: F, t93041: F, t93043: F, t93045: F, t93049: F, t93051: F, t93055: F) -> F {
    let t95684 = F::new(0.28900264064772933812e-2) * t93020;
    let t95698 = -t95684 - F::new(0.20579528696673473747e-1) * t93022 + F::new(0.30492001685571196935e-3) * t93026 + F::new(0.12004725073059526352e-1) * t93028 - F::new(0.68598428988911579154e-3) * t93031 + F::new(0.16262400898971305032e-2) * t93035 + F::new(0.51448821741683684367e-1) * t93037 + F::new(0.51448821741683684367e-2) * t93039 - F::new(0.85748036236139473944e-3) * t93041 - F::new(0.15246000842785598468e-3) * t93043 + F::new(0.12004725073059526352e-1) * t93045 - F::new(0.68026775414003982662e-1) * t93049 - F::new(0.85748036236139473944e-3) * t93051 - F::new(0.24009450146119052704e-1) * t93055;
    t95698
}
