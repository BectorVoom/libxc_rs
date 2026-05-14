//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1000/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1000<F: Float>(t4680: F, t7413: F, t9648: F, t1815: F, t1983: F, t30127: F, t7586: F, t31350: F, t6343: F, t30866: F, t30868: F, t30872: F, t30874: F, t30878: F, t30880: F, t30883: F, t39615: F, t39617: F, t39620: F, t39623: F, t39626: F, t39629: F, t39632: F, t39640: F) -> (F,) {
    let t39643 = t7413 * t4680 * t9648;
    let t39647 = t30127 * t7586 * t1983 * t1815;
    let t39649 = t31350 * t6343;
    let t39651 = 7.0 / 288.0 * t39615 - t39617 / 96.0 - t39620 / 64.0 + t39623 / 96.0 + 0.1528125e-1 * t39626 - t39629 / 4.0 + t39632 / 48.0 - 0.85748036236139473944e-3 * t30866 + 0.22675591804667994222e-1 * t30868 - 0.22675591804667994222e-1 * t30872 + 0.16006300097412701803e-1 * t30874 - 0.80031500487063509016e-2 * t30878 + 0.45017719023973223821e-2 * t30880 + t30883 + 0.21437009059034868486e-3 * t39640 + 0.21437009059034868486e-3 * t39643 + 0.14291339372689912324e-3 * t39647 - 0.85748036236139473945e-2 * t39649;
    (t39651,)
}
