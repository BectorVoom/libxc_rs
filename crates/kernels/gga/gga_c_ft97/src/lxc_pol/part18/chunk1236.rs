//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1236/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1236<F: Float>(t23084: F, t3238: F, t1882: F, t26284: F, t26288: F, t26446: F, t8392: F, t1851: F, t5617: F, t26163: F, t26168: F, t1637: F, t6526: F, t89: F, t102394: F, t11490: F, t11902: F, t1647: F, t1901: F, t23331: F, t23335: F, t26162: F, t26372: F, t3214: F, t3219: F, t446: F, t47007: F, t47222: F, t5743: F, t6538: F, t83: F, t8417: F, t8557: F, t91817: F) -> (F, F) {
    let t102701 = t3238 * t23084;
    let t102706 = 2.0 / 9.0 * t1882 * t26284;
    let t102708 = 2.0 / 9.0 * t1882 * t26288;
    let t102723 = 2.0 / 27.0 * t8392 * t26446;
    let t102724 = t1851 * t5617;
    let t102730 = 4.0 / 9.0 * t8392 * t26163;
    let t102732 = 4.0 / 9.0 * t8392 * t26168;
    let t102743 = t89 * t1637 * t6526;
    let t102748 = -t446 * t83 * t102701 / 3.0 + t102706 + t102708 - 4.0 / 3.0 * t1901 * t11490 * t91817 * t3214 - 4.0 * t1901 * t26372 * t8417 * t5743 * t3219 + 2.0 / 9.0 * t1901 * t8557 * t6538 * t1647 + t102723 - 4.0 / 3.0 * t1901 * t11490 * t102724 * t3219 + t102730 + t102732 - 4.0 / 3.0 * t1901 * t47007 * t26162 + t1901 * t11902 * t23331 / 9.0 + 2.0 / 27.0 * t1901 * t47222 * t23335 + 4.0 / 27.0 * t102743 + 4.0 / 3.0 * t446 * t83 * t102394;
    (t102701, t102748)
}
