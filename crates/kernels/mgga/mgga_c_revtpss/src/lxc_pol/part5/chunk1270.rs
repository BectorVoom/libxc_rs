//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1270/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1270<F: Float>(t22035: F, t22065: F, t22105: F, t22140: F, t22153: F, t22176: F, t22284: F, t22304: F, t6862: F, t72: F, t686: F, t10023: F, t1385: F, t6888: F, t10070: F, t10074: F, t1399: F, t14191: F, t14193: F, t14203: F, t14209: F, t14255: F, t1883: F, t213: F, t21981: F, t22005: F, t22009: F, t22016: F, t4118: F, t546: F, t5659: F, t5675: F, t5745: F, t5755: F, t5767: F, t6874: F, t820: F) -> (F, F) {
    let t22307 = t22035 + t22065 + t22105 + t22140 + t22153 + t22176 + t22284 + t22304;
    let t22314 = t6862 * t72;
    let t22315 = t22314 * t686;
    let t22316 = t10023 * t22315;
    let t22321 = t1385 * t6888;
    let t22325 = -0.13170898365871023197e1 * t820 * t14255 * t1883 - 0.13170898365871023197e1 * t820 * t5767 * t5659 - 0.65854491829355115987e0 * t5755 * t22005 * t1399 + 0.13170898365871023197e1 * t5745 * t22009 * t5675 + 0.26341796731742046394e1 * t5745 * t21981 * t5675 - 0.39512695097613069591e1 * t14193 * t22005 * t22016 + 0.65854491829355115987e0 * t213 * t546 * t22307 - 0.65854491829355115987e0 * t820 * t4118 * t6874 + 0.19514881078765566037e-1 * t22316 - t14191 - 0.13009920719177044025e-2 * t14203 + t14209 - 0.73171657588172351096e-2 * t10070 + 0.65049603595885220126e-3 * t10074 - 0.65854491829355115987e0 * t820 * t22321 * t1399;
    (t22307, t22325)
}
