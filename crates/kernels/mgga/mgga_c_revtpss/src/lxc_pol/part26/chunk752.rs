//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 752/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk752<F: Float>(t3829: F, t4011: F, t547: F, t807: F, t2237: F, t240: F, t550: F, t816: F, t1379: F, t2689: F, t3952: F, t1413: F, t3889: F) -> (F, F, F, F, F, F, F) {
    let t9703 = t4011 * t3829;
    let t9704 = t547 * t9703;
    let t9705 = t807 * t9704;
    let t9707 = t2237 * t240;
    let t9709 = t9707 * t550 * t816;
    let t9711 = F::new(0.12846167376791569079e-2) * t1379 * t9709;
    let t9712 = t2689 * t3952;
    let t9714 = t1413 * t3889;
    (t9703, t9705, t9707, t9709, t9711, t9712, t9714)
}
