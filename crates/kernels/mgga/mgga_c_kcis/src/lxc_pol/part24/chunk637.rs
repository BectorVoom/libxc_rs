//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 637/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk637<F: Float>(t1122: F, t7726: F, t303: F, t355: F, t359: F, t982: F, t342: F, t1134: F, t356: F, t2173: F, t2175: F, t7687: F, t7690: F, t7693: F, t7696: F, t7701: F, t7703: F, t7706: F, t7711: F, t7717: F, t7721: F, t7724: F) -> (F, F, F, F, F, F, F, F) {
    let t7727 = t7726 * t1122;
    let t7728 = t303 * t7727;
    let t7731 = t355 * t982 * t359;
    let t7732 = t342 * t7731;
    let t7733 = t303 * t7732;
    let t7735 = t356 * t1134;
    let t7736 = t303 * t7735;
    let t7738 = -0.69505208333333333333e-3 * t7687 * t2175 + 0.92754700520833333333e-4 * t7690 * t7693 + 0.18534722222222222222e-2 * t7696 * t2175 - t7701 - 0.23168402777777777778e-3 * t7703 * t7706 + 0.69505208333333333333e-3 * t2173 * t7711 + 0.69505208333333333333e-3 * t2173 * t7693 + t7717 + 0.16581944444444444444e-2 * t7721 + 0.24872916666666666666e-2 * t7724 - 0.24872916666666666666e-2 * t7728 - 0.66327777777777777776e-2 * t7733 + 0.16581944444444444444e-2 * t7736;
    (t7727, t7728, t7731, t7732, t7733, t7735, t7736, t7738)
}
