//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 761/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk761(t12938: f64, t9438: f64, t587: f64, t2366: f64, t3338: f64, t2365: f64, t1429: f64, t10418: f64, t901: f64, t12528: f64, t12542: f64, t10608: f64, t3177: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12939 = t9438 * t12938;
    let t12940 = t587 * t12939;
    let t12941 = 0.15976219147466979032e-1_f64 * t12940;
    let t12942 = t2366 * t3338;
    let t12943 = t2365 * t12942;
    let t12944 = t1429 * t12943;
    let t12946 = t10418 * t901;
    let t12948 = 0.11502877786176224903e1_f64 * t12528;
    let t12952 = 0.19171462976960374838e1_f64 * t12542;
    let t12953 = t10608 * t3177;
    (t12939, t12941, t12942, t12943, t12944, t12946, t12948, t12952, t12953)
}
