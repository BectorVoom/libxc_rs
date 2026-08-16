//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 852/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk852<F: Float>(t12328: F, t555: F, t10027: F, t541: F, t1361: F, t2690: F, t1336: F, t241: F, t67: F, t6924: F, t1339: F, t3788: F, t835: F) -> (F, F, F, F, F, F) {
    let t12330 = F::cast_from(595.0_f64) / F::cast_from(10368.0_f64) * t555 * t12328;
    let t12335 = F::cast_from(455.0_f64) / F::cast_from(1296.0_f64) * t10027 * t541;
    let t12344 = t1361 * t2690;
    let t12345 = t1336 * t12344;
    let t12351 = t241 * t6924 * t67;
    let t12364 = t1339 * t2690;
    let t12365 = t1336 * t12364;
    let t12384 = t3788 * t835;
    (t12330, t12335, t12345, t12351, t12365, t12384)
}
