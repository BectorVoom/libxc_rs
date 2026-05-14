//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1229/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1229<F: Float>(t6027: F, t679: F, t172: F, t30674: F, t30676: F, t30671: F, t123165: F, t123169: F, t123173: F, t123195: F, t1417: F, t1701: F, t17851: F, t17937: F, t231: F, t24311: F, t24346: F, t27494: F, t27546: F, t27557: F, t27584: F, t3774: F, t3817: F, t5005: F, t52861: F, t6045: F, t709: F, t96619: F, t96630: F, t96739: F) -> (F, F, F) {
    let t123459 = t6027 * t679;
    let t123503 = t30674 * t172 * t30676;
    let t123504 = t30671 * t123503;
    let t123523 = -t96739 - 0.2370952259137005195e-1 * t1417 * t1701 * t27494 * t3817 + 0.46509801892875584e-1 * t24346 * t17937 - 0.81118562704294997117e-4 * t52861 * t123195 - 0.13519760450715832853e-3 * t17851 * t96619 - 0.3472439437926143696e-6 * t123504 + 0.76612330055555555556e-1 * t27546 * t6045 * t231 * t5005 * t709 + 0.10330921273483950306e-5 * t3774 * t24311 * t123165 - 0.3443640424494650102e-5 * t3774 * t24311 * t123169 + 0.28677218675336554254e-7 * t3774 * t96630 * t123173 + 0.55136259934963963186e-4 * t3774 * t27584 * t27557;
    (t123459, t123503, t123523)
}
