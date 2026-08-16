//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1133/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1133<F: Float>(t1060: F, t18088: F, t1022: F, t360: F, t6739: F, t5928: F, t1049: F, t5866: F, t11066: F, t3201: F, t4649: F, t1629: F) -> (F, F, F, F, F) {
    let t18089 = t18088 * t1060;
    let t18093 = t6739 * t1022 * t360;
    let t18094 = t5928 * t18093;
    let t18099 = t1049 * t5866;
    let t18100 = t18099 * t1060;
    let t18103 = t11066 * t1022;
    let t18104 = t5928 * t18103;
    let t18107 = t3201 * t4649;
    let t18108 = t1629 * t18107;
    (t18089, t18094, t18100, t18104, t18108)
}
