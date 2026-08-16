//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1384/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1384(t300: f64, t77343: f64, t77370: f64, t77390: f64, t77471: f64, t10629: f64, t2932: f64, t76637: f64, t959: f64, t2929: f64, t77139: f64, t77153: f64, t77157: f64, t77159: f64, t77224: f64, t77226: f64, t77229: f64, t77232: f64, t77236: f64, t77470: f64) -> (f64, f64, f64, f64) {
    let t77474 = t300 * (t77343 + t77370 + t77390 + t77471);
    let t77478 = 0.6233709278045326953e3_f64 * t959 * t10629 * t76637 * t2932;
    let t77482 = 0.51947577317044391277e2_f64 * t959 * t2929 * t77139 * t2932;
    let t77483 = -t77153 + t77157 + t77159 - t77224 + t77226 - t77229 - t77232 + t77236 + t77474 - t77478 - t77470 - t77482;
    (t77474, t77478, t77482, t77483)
}
