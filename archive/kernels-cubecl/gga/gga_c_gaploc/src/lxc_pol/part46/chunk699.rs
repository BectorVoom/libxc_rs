//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 699/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk699<F: Float>(t12508: F, t12510: F, t12512: F, t9439: F, t986: F, t9438: F, t587: F, t2366: F, t3338: F, t2365: F, t1429: F, t10418: F, t901: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12935 = F::cast_from(0.29792074959875355558e-1_f64) * t12508;
    let t12936 = F::cast_from(0.29792074959875355558e-1_f64) * t12510;
    let t12937 = F::cast_from(0.29792074959875355558e-1_f64) * t12512;
    let t12938 = t9439 * t986;
    let t12939 = t9438 * t12938;
    let t12940 = t587 * t12939;
    let t12941 = F::cast_from(0.15976219147466979032e-1_f64) * t12940;
    let t12942 = t2366 * t3338;
    let t12943 = t2365 * t12942;
    let t12944 = t1429 * t12943;
    let t12946 = t10418 * t901;
    (t12935, t12936, t12937, t12938, t12939, t12941, t12942, t12943, t12944, t12946)
}
