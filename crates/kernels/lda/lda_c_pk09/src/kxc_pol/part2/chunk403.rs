//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 403/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk403<F: Float>(t1800: F, t2000: F, t1972: F, t477: F, t443: F, t485: F, t1754: F, t1765: F, t1684: F, t1735: F, t1732: F, t1738: F, t1762: F, t1769: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2001 = t2000 * t1800;
    let t2002 = F::cast_from(22.07984838129906_f64) * t2001;
    let t2003 = t1972 * t477;
    let t2005 = t443 * t443;
    let t2006 = F::new(1.0) / t2005;
    let t2007 = t2006 * t485;
    let t2008 = F::cast_from(0.22687409291590604_f64) * t1754;
    let t2010 = F::cast_from(0.07562469763863536_f64) * t1765;
    let t2012 = F::cast_from(0.04525483399593904_f64) * t1684;
    let t2014 = F::cast_from(0.015084944665313014_f64) * t1735;
    let t2016 = t2008 - F::cast_from(0.22687409291590604_f64) * t1762 + t2010 + F::cast_from(0.22687409291590604_f64) * t1769 + t2012 - F::cast_from(0.04525483399593904_f64) * t1732 + t2014 + F::cast_from(0.04525483399593904_f64) * t1738;
    (t2001, t2002, t2003, t2005, t2006, t2007, t2008, t2010, t2012, t2014, t2016)
}
