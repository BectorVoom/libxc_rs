//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2692/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2692(t12250: f64, t6414: f64, t1824: f64, t6434: f64, t1336: f64, t1352: f64, t16047: f64, t1825: f64, t19654: f64, t19657: f64, t19744: f64, t19748: f64, t19815: f64, t20490: f64, t20568: f64, t20622: f64, t3777: f64, t3901: f64, t40492: f64, t5250: f64, t5287: f64, t5334: f64, t5335: f64, t5344: f64, t5349: f64, t57618: f64, t74174: f64, t74941: f64) -> (f64, f64) {
    let t75008 = t12250 * t6414;
    let t75026 = t6434 * t1824;
    let t75053 = -3.0_f64 * t1336 * t19657 * t5287 - 6.0_f64 * t1336 * t20490 * t40492 - t1336 * t20568 * t3901 - 3.0_f64 * t1352 * t5344 * t75026 - 18.0_f64 * t16047 * t19744 * t74941 - 3.0_f64 * t1825 * t5344 * t57618 + 6.0_f64 * t5250 * t5334 * t75026 + 6.0_f64 * t5334 * t5335 * t74174 + 18.0_f64 * t19654 * t19748 - 3.0_f64 * t19815 * t5349 - 6.0_f64 * t20622 * t3777;
    (t75008, t75053)
}
