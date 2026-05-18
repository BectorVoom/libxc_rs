//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 403/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk403<F: Float>(t265: F, t393: F, t1032: F, t1078: F, t1976: F, t359: F, t1978: F, t1983: F, t342: F, t1962: F, t207: F, t198: F, t892: F, t1102: F, t336: F) -> (F, F, F, F, F, F) {
    let t394 = t265 < t393;
    let t1984 = t1032 * t1078;
    let t1985 = t359 * t1976;
    let t1986 = t1984 * t1985;
    let t1989 = F::new(0.65854491829355115987e0) * t342 * t1978 - F::new(0.4336814094102599731e0) * t1983 * t1986;
    let t1993 = t207 * t1962;
    let t1995 = t198 * t1993 * t892;
    let t1996 = piecewise3::<f64>(t394, t198 * t336 * t1989 * t1102, t1995);
    (t1984, t1986, t1989, t1993, t1995, t1996)
}
