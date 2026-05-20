//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2970/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2970<F: Float>(t4711: F, t64504: F, t981: F, t23811: F, t300: F, t983: F, t52238: F, t78423: F, t18898: F, t52459: F, t15258: F, t19133: F) -> (F, F, F, F, F) {
    let t78703 = F::cast_from(0.51947577317044391277e2_f64) * t981 * t64504 * t4711;
    let t78704 = t300 * t23811;
    let t78706 = F::cast_from(0.5848223622634646207e0_f64) * t78704 * t983;
    let t78709 = F::cast_from(0.31168546390226634766e3_f64) * t52238 * t4711 * t78423;
    let t78712 = F::cast_from(0.30762056574649219974e4_f64) * t981 * t18898 * t52459;
    let t78715 = F::cast_from(0.31168546390226634765e3_f64) * t981 * t19133 * t15258;
    (t78703, t78706, t78709, t78712, t78715)
}
