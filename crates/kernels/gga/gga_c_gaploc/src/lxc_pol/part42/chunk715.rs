//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 715/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk715<F: Float>(t44294: F, t6508: F, t1358: F, t6507: F, t2339: F, t35918: F, t42581: F, t42587: F, t42590: F, t11182: F, t2317: F, t6525: F, t35900: F, t883: F, t2761: F, t9074: F) -> (F, F, F, F, F, F, F, F, F) {
    let t44295 = t6508 * t44294;
    let t44298 = 0.63233348079280332442e-2 * t1358 * t6507 * t44295;
    let t44301 = 0.22131671827748116354e-1 * t1358 * t35918 * t2339;
    let t44302 = 0.18970004423784099733e-1 * t42581;
    let t44305 = 0.142275033178380748e-1 * t42587;
    let t44306 = 0.142275033178380748e-1 * t42590;
    let t44308 = t6525 * t11182 * t2317;
    let t44309 = 0.11856252764865062333e-2 * t44308;
    let t44310 = t883 * t35900;
    let t44312 = t9074 * t2761 * t44310;
    (t44295, t44298, t44301, t44302, t44305, t44306, t44309, t44310, t44312)
}
