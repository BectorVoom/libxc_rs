//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1172/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1172<F: Float>(t100533: F, t31999: F, t8513: F, t1047: F, t1096: F, t120281: F, t120513: F, t120602: F, t120664: F, t120676: F, t120696: F, t126725: F, t126903: F, t1984: F, t27627: F, t27652: F, t31892: F, t31897: F, t31903: F, t31943: F, t31959: F, t31986: F, t33791: F, t33792: F, t359: F, t4742: F, t4772: F, t4976: F, t7135: F, t7821: F, t8521: F, t8524: F, t999: F) -> F {
    let t127016 = t8513 * t100533 * t31999;
    let t127035 = -F::new(0.51407763898592117355e1) * t120602 * t31892 * t33791 * t999 - F::new(0.10038921514126388266e-2) * t120696 + F::new(0.17135921299530705785e1) * t120281 * t33792 - F::new(0.1859366460452550541e-3) * t126725 * t8521 * t8524 - F::new(0.17347256376410398924e1) * t120513 * t126903 * t27652 + F::new(0.17347256376410398924e1) * t120664 * t126903 * t4976 - F::new(0.17347256376410398924e1) * t120676 * t27627 - F::new(0.3718732920905101082e-3) * t127016 * t1047 - F::new(0.3427184259906141157e1) * t31903 * t31892 * t7821 * t7135 - F::new(0.51407763898592117355e1) * t31897 * t31959 * t33791 * t1096 + F::new(0.17347256376410398924e1) * t31943 * t1984 * t359 * t4772 - F::new(0.17347256376410398924e1) * t31986 * t1984 * t359 * t4742;
    t127035
}
