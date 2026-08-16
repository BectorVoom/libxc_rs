//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 695/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk695<F: Float>(t12450: F, t12508: F, t12510: F, t12512: F, t9439: F, t986: F, t9438: F, t587: F, t12528: F, t12542: F, t10608: F, t3177: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12930 = F::cast_from(0.63904876589867916127e-1_f64) * t12450;
    let t12935 = F::cast_from(0.29792074959875355558e-1_f64) * t12508;
    let t12936 = F::cast_from(0.29792074959875355558e-1_f64) * t12510;
    let t12937 = F::cast_from(0.29792074959875355558e-1_f64) * t12512;
    let t12938 = t9439 * t986;
    let t12939 = t9438 * t12938;
    let t12940 = t587 * t12939;
    let t12941 = F::cast_from(0.15976219147466979032e-1_f64) * t12940;
    let t12948 = F::cast_from(0.11502877786176224903e1_f64) * t12528;
    let t12952 = F::cast_from(0.19171462976960374838e1_f64) * t12542;
    let t12953 = t10608 * t3177;
    (t12930, t12935, t12936, t12937, t12938, t12939, t12941, t12948, t12952, t12953)
}
