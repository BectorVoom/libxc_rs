//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1104/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1104<F: Float>(t14230: F, t26079: F, t120991: F, t121019: F, t5676: F, t121018: F, t5674: F, t94396: F, t121077: F, t121139: F, t121144: F, t121147: F, t121168: F, t121178: F, t121182: F, t121188: F, t125717: F, t125721: F, t125729: F, t125732: F, t14224: F, t1444: F, t27852: F, t27980: F, t32233: F, t32250: F, t32252: F, t32258: F, t32719: F, t33946: F, t33955: F, t34231: F, t7274: F, t7920: F, t8706: F) -> F {
    let t125734 = t26079 * t14230;
    let t125749 = t120991 * t121019 * t5676;
    let t125753 = t121018 * t121019 * t5674 * t94396;
    let t125763 = -F::new(0.11156198762715303246e-2) * t125717 - F::new(0.3718732920905101082e-2) * t125721 - F::new(0.22847895066040941046e1) * t32719 * t27980 * t27852 - F::new(0.11423947533020470523e1) * t32719 * t121077 * t14224 + F::new(0.42839803248826764462e-1) * t125729 + t121139 - t121144 - F::new(0.34708173928447610099e-2) * t125732 - F::new(0.17347256376410398924e1) * t32233 * t125734 - F::new(0.34271842599061411569e1) * t8706 * t32250 * t33955 * t1444 + F::new(0.25702851531048074406e-1) * t121147 - F::new(0.11423947533020470523e1) * t34231 * t32258 + F::new(0.6854368519812282314e1) * t8706 * t121188 * t33946 * t1444 - F::new(0.11156198762715303246e-2) * t125749 - F::new(0.7437465841810202164e-3) * t125753 - t121168 - F::new(0.17135921299530705785e1) * t34231 * t32252 - F::new(0.3427184259906141157e1) * t8706 * t32250 * t7920 * t7274 - F::new(0.33467254597718846885e-4) * t121178 - F::new(0.14456046980341999104e-1) * t121182;
    t125763
}
