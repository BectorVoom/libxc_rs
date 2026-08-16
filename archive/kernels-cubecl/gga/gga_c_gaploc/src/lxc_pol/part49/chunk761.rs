//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 761/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk761<F: Float>(t12938: F, t9438: F, t587: F, t2366: F, t3338: F, t2365: F, t1429: F, t10418: F, t901: F, t12528: F, t12542: F, t10608: F, t3177: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12939 = t9438 * t12938;
    let t12940 = t587 * t12939;
    let t12941 = F::cast_from(0.15976219147466979032e-1_f64) * t12940;
    let t12942 = t2366 * t3338;
    let t12943 = t2365 * t12942;
    let t12944 = t1429 * t12943;
    let t12946 = t10418 * t901;
    let t12948 = F::cast_from(0.11502877786176224903e1_f64) * t12528;
    let t12952 = F::cast_from(0.19171462976960374838e1_f64) * t12542;
    let t12953 = t10608 * t3177;
    (t12939, t12941, t12942, t12943, t12944, t12946, t12948, t12952, t12953)
}
