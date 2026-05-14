//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 625/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk625<F: Float>(t12922: F, t8352: F, t2778: F, t3116: F, t1445: F, t574: F, t12446: F, t12450: F, t12508: F, t12510: F, t12512: F, t9439: F, t986: F, t9438: F, t587: F, t12528: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12924 = 0.42900587942220512003e1 * t8352 * t12922;
    let t12925 = t2778 * t3116;
    let t12926 = t1445 * t12925;
    let t12928 = 0.46011511144704899612e1 * t574 * t12926;
    let t12929 = 0.63904876589867916127e-1 * t12446;
    let t12930 = 0.63904876589867916127e-1 * t12450;
    let t12935 = 0.29792074959875355558e-1 * t12508;
    let t12936 = 0.29792074959875355558e-1 * t12510;
    let t12937 = 0.29792074959875355558e-1 * t12512;
    let t12938 = t9439 * t986;
    let t12939 = t9438 * t12938;
    let t12940 = t587 * t12939;
    let t12941 = 0.15976219147466979032e-1 * t12940;
    let t12948 = 0.11502877786176224903e1 * t12528;
    (t12924, t12925, t12926, t12928, t12929, t12930, t12935, t12936, t12937, t12938, t12939, t12941, t12948)
}
